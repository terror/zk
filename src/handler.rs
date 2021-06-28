use crate::common::*;

pub struct Handler {
  pub config: Config,
}

impl Handler {
  /// Creates a new note with the specified `name` in the Zettelkasten directory with an
  /// appropriate prefix in addition to writing the default YAML frontmatter.
  pub fn new(&self, name: &str) -> Result<(), Error> {
    let mut file = File::create(&self.config.path.expand().join(Note::generate_name(name)))
      .context(error::Io)?;

    file
      .write_all(format!("---\nname: {}\n---\n", name).as_bytes())
      .context(error::Io)?;

    self.open(name)?;

    Ok(())
  }

  /// Opens a note given a `name` using the editor specified in the
  /// configuration file. If there are multiple notes present with the
  /// same `name`, the user will be prompted interactively to choose
  /// which file(s) is/are desired to be opened.
  pub fn open(&self, name: &str) -> Result<(), Error> {
    let mut candidates = vec![];

    for entry in WalkDir::new(&self.config.path.expand()) {
      let entry = entry.unwrap();

      let filename = entry
        .path()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

      if let Some(candidate) = Note::retrieve(&filename, Part::Name) {
        if candidate == name {
          candidates.push(format!("{}.md", filename));
        }
      }
    }

    if candidates.len() == 0 {
      println!("No note with name {} found.", name);
      return Ok(());
    }

    if candidates.len() == 1 {
      let filename = candidates.first().unwrap();
      Command::new(&self.config.editor)
        .arg(&self.config.path.expand().join(filename))
        .status()
        .context(error::Io)?;
      return Ok(());
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("There exist multiple notes with the name `{}`, please choose which one you would like to open:", name))
        .items(&candidates[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
      println!("No note selected.");
      return Ok(());
    }

    for selection in selections {
      let filename = &candidates[selection];
      let path = Path::join(&self.config.path.expand(), Path::new(&filename.to_string()));
      Command::new(&self.config.editor)
        .arg(&path)
        .status()
        .context(error::Io)?;
    }

    Ok(())
  }

  /// Links two notes together. This entails checking and modifying both notes'
  /// YAML frontmatter to ensure a link is created.
  pub fn link(&self, _left: &str, _right: &str) -> Result<(), Error> {
    Ok(())
  }

  /// Finds all notes given a `tag`. This method invoke `skim` using the
  /// names of the notes that contain `tag` within the frontmatter.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    let path = &self.config.path.expand();

    let notes = Note::all(&path)?;

    let mut candidates = vec![];

    for note in notes {
      if let Some(tags) = note.matter["tags"].as_vec() {
        if tags.contains(&yaml_rust::Yaml::from_str(&tag)) {
          candidates.push(note.filename());
        }
      }
    }

    let mut input = String::new();

    for candidate in &candidates {
      input.push_str(&format!("{}\n", candidate));
    }

    let options = SkimOptions::default();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
      .map(|out| out.selected_items)
      .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
      if let Some(name) = Note::retrieve(&item.output().to_string(), Part::Name) {
        self.open(name)?;
      }
    }

    Ok(())
  }

  /// Starts a fuzzy search in the Zettelkasten directory.
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub fn search(&self) -> Result<(), Error> {
    env::set_current_dir(&self.config.path.expand()).unwrap();

    let options = SkimOptions::default();

    let selected_items = Skim::run_with(&options, None)
      .map(|out| out.selected_items)
      .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
      if let Some(name) = Note::retrieve(&item.output().to_string(), Part::Name) {
        self.open(name)?;
      }
    }

    Ok(())
  }
}
