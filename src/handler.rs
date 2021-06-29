use crate::common::*;

pub struct Handler {
  pub editor:    String,
  pub directory: Directory,
}

impl Handler {
  pub fn new(editor: String, directory: Directory) -> Self {
    Self { editor, directory }
  }

  /// Creates a new note with the specified `name` in the Zettelkasten directory
  /// with an appropriate prefix in addition to writing the default YAML
  /// frontmatter.
  pub fn create(&self, name: &str) -> Result<(), Error> {
    let mut file =
      File::create(&self.directory.path.join(Note::generate_name(name))).context(error::Io)?;

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
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, open it and return
      if candidates.len() == 1 {
        let filename = candidates.first().unwrap();
        Command::new(&self.editor)
          .arg(&self.directory.path.join(filename))
          .status()
          .context(error::Io)?;
        return Ok(());
      }

      let prompt = Prompt::new(
        format!(
          "There exist multiple notes with the name `{}`, please choose which one you would like \
           to open:",
          name
        ),
        candidates,
      );

      // prompt the user with each candidate note
      if let Some(selections) = prompt.interact() {
        for selection in selections {
          let path = Path::join(&self.directory.path, Path::new(&selection.to_string()));
          Command::new(&self.editor)
            .arg(&path)
            .status()
            .context(error::Io)?;
        }

        return Ok(());
      }
    }

    println!("No note with name `{}` was found.", name);
    Ok(())
  }

  /// Links two notes together. This entails checking and modifying both notes'
  /// YAML frontmatter to ensure a link is created.
  ///
  /// Some things to consider:
  ///
  /// - Prompt the user if `left` or `right` exist more than once in the
  /// storage location
  ///
  /// - Check if `left` and `right` do not already contain each other in
  /// the yaml frontmatter
  pub fn link(&self, left: &str, right: &str) -> Result<(), Error> {
    let left_candidates = self.directory.find(left);

    let right_candidates = self.directory.find(right);

    if left_candidates.is_none() || right_candidates.is_none() {
      println!("Both notes must exist in order to be linked.");
      return Ok(());
    }

    let left_prompt = Prompt::new(
      format!(
        "There exists multiple notes with name: {}, please choose which one you would like to use:",
        left
      ),
      left_candidates.unwrap(),
    );

    let right_prompt = Prompt::new(
      format!(
        "There exists multiple notes with name: {}, please choose which one you would like to use:",
        left
      ),
      right_candidates.unwrap(),
    );

    let left = left_prompt.interact_choose_first();

    let right = right_prompt.interact_choose_first();

    if left.is_none() || right.is_none() {
      println!("You must choose two notes in order to link them together.");
      return Ok(());
    }

    let left = Note::from(self.directory.path.join(left.unwrap()));

    let right = Note::from(self.directory.path.join(right.unwrap()));

    // At this point we have two notes that exist and need to be linked
    // together. This requires checking and/or modifying the YAML frontmatter
    // to ensure that the two notes here are linked.
    //
    // Initial approach:
    //
    // Rebuild the YAML links array somehow and write the frontmatter +
    // content to each note.
    //
    // Some cases to think about:
    //  - The notes are already linked (in this case, do nothing)
    //  - Only one note links the other (in this case, add the missing
    //  link)
    //  - The notes aren't linked (in this case, add them both)
    //
    // Either way, the linking of notes will require some modification
    // to the notes YAML frontmatter and subsequently writing it back to
    // the file.

    if left.has_link(&right.filename()) && right.has_link(&left.filename()) {
      println!("The two notes are already linked.");
      return Ok(());
    }

    if !left.has_link(&right.filename()) {
      let contents = fs::read_to_string(self.directory.path.join(left.filename())).unwrap();
      let frontmatter = Yaml::from_str(&contents);
      println!("{:?}", frontmatter);
    }

    if !right.has_link(&left.filename()) {
      let contents = fs::read_to_string(self.directory.path.join(right.filename())).unwrap();
      let frontmatter = Yaml::from_str(&contents);
      println!("{:?}", frontmatter);
    }

    println!("{} <-> {}", left.filename(), right.filename());
    Ok(())
  }

  /// Finds all notes given a `tag`. This method invoke `skim` using the
  /// names of the notes that contain `tag` within the frontmatter.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    let notes = self.directory.fetch();

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
    env::set_current_dir(&self.directory.path).unwrap();

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

  /// Writes the current Zettelkasten storage location to stdout.
  pub fn dir(&self) -> Result<(), Error> {
    println!("{}", self.directory.path.expand().display());
    Ok(())
  }

  /// Outputs a notes contents to stdout using the `termimad` library.
  /// This is meant as an easy way to view a Zettels contents without
  /// having to call `open`. User will be prompted interactively if multiple
  /// notes exist with the same `name`.
  pub fn preview(&self, name: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, preview it and return
      if candidates.len() == 1 {
        let filename = candidates.first().unwrap();
        let contents = fs::read_to_string(&self.directory.path.join(filename)).unwrap();
        termimad::print_text(&contents);
        return Ok(());
      }

      let prompt = Prompt::new(
        format!(
          "There exist multiple notes with the name `{}`, please choose which one you would like \
           to preview:",
          name
        ),
        candidates,
      );

      // preview each candidate note
      if let Some(selections) = prompt.interact() {
        for selection in selections {
          let path = Path::join(&self.directory.path, Path::new(&selection.to_string()));
          let contents = fs::read_to_string(&path).unwrap();
          termimad::print_text(&contents);
        }
      }

      return Ok(());
    }

    println!("No note with name `{}` was found.", name);
    Ok(())
  }

  /// Removes an existing note in the Zettelkasten directory. This will
  /// also prompt the user if more than one note exists with `name`.
  pub fn remove(&self, name: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, delete it and return
      if candidates.len() == 1 {
        let filename = candidates.first().unwrap();
        fs::remove_file(&self.directory.path.join(filename)).unwrap();
        return Ok(());
      }

      let prompt = Prompt::new(
        format!(
          "There exist multiple notes with the name `{}`, please choose which one you would like \
           to delete:",
          name
        ),
        candidates,
      );

      // delete each candidate note
      if let Some(selections) = prompt.interact() {
        for selection in selections {
          let path = Path::join(&self.directory.path, Path::new(&selection.to_string()));
          fs::remove_file(&path).unwrap();
        }
      }

      return Ok(());
    }

    println!("No note with name `{}` was found.", name);
    Ok(())
  }
}
