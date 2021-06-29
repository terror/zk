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
      File::create(&self.directory.path.join(NoteId::new(name).to_string())).context(error::Io)?;

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
        let filename = candidates.first().unwrap().id.to_string();
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
        candidates
          .iter()
          .map(|candidate| candidate.id.to_string())
          .collect(),
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
    if let (Some(l), Some(r)) = (self.directory.find(left), self.directory.find(right)) {
      let left_prompt = Prompt::new(
        format!(
          "There exists multiple notes with name: {}, please choose which one you would like to \
           use:",
          left
        ),
        l.iter().map(|note| note.id.to_string()).collect(),
      );

      let right_prompt = Prompt::new(
        format!(
          "There exists multiple notes with name: {}, please choose which one you would like to \
           use:",
          right
        ),
        r.iter().map(|note| note.id.to_string()).collect(),
      );

      if let (Some(l), Some(r)) = (left_prompt.interact(), right_prompt.interact()) {
        let left = Note::from(self.directory.path.join(l.first().unwrap()));

        let right = Note::from(self.directory.path.join(r.first().unwrap()));

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

        if left.has_link(&right.id.to_string()) && right.has_link(&left.id.to_string()) {
          println!("The two notes are already linked.");
          return Ok(());
        }

        if !left.has_link(&right.id.to_string()) {
          left.add_link(&right.id.to_string())?;
        }

        if !right.has_link(&left.id.to_string()) {
          right.add_link(&left.id.to_string())?;
        }

        println!("{} <-> {}", left.id, right.id);
      } else {
        println!("You must choose two notes in order to link them together.");
        return Ok(());
      }
    } else {
      println!("Both notes must exist in order to be linked.");
      return Ok(());
    }

    Ok(())
  }

  /// Finds all notes given a `tag`. This method invokes `skim` using the
  /// names of the notes that contain `tag` within the frontmatter and
  /// attempts to open each selected item.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find_by_tag(tag) {
      let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .preview(Some(""))
        .build()
        .unwrap();

      let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

      for candidate in candidates {
        tx.send(Arc::new(SearchItem {
          text: candidate.id.to_string(),
          path: candidate.path,
        }))
        .unwrap();
      }

      drop(tx);

      let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

      for item in selected_items.iter() {
        if let Some(id) = NoteId::parse(&item.output().to_string()) {
          self.open(&id.name)?;
        }
      }

      return Ok(());
    }

    println!("No notes exist with the tag `{}`.", tag);
    Ok(())
  }

  /// Starts a fuzzy search using note id's in the Zettelkasten directory
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub fn search(&self) -> Result<(), Error> {
    let options = SkimOptionsBuilder::default()
      .height(Some("50%"))
      .multi(true)
      .preview(Some(""))
      .build()
      .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for note in self.directory.notes() {
      tx.send(Arc::new(SearchItem {
        text: note.id.to_string(),
        path: note.path,
      }))
      .unwrap();
    }

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        self.open(&id.name)?;
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
        let filename = candidates.first().unwrap().id.to_string();
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
        candidates
          .iter()
          .map(|candidate| candidate.id.to_string())
          .collect(),
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
        let filename = candidates.first().unwrap().id.to_string();
        fs::remove_file(&self.directory.path.join(filename)).unwrap();
        return Ok(());
      }

      let prompt = Prompt::new(
        format!(
          "There exist multiple notes with the name `{}`, please choose which one you would like \
           to delete:",
          name
        ),
        candidates
          .iter()
          .map(|candidate| candidate.id.to_string())
          .collect(),
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

  /// Removes a link between two existing notes
  pub fn remove_link(&self, left: &str, right: &str) -> Result<(), Error> {
    if let (Some(l), Some(r)) = (self.directory.find(left), self.directory.find(right)) {
      let left_prompt = Prompt::new(
        format!(
          "There exists multiple notes with name: {}, please choose which one you would like to \
           use:",
          left
        ),
        l.iter().map(|note| note.id.to_string()).collect(),
      );

      let right_prompt = Prompt::new(
        format!(
          "There exists multiple notes with name: {}, please choose which one you would like to \
           use:",
          right
        ),
        r.iter().map(|note| note.id.to_string()).collect(),
      );

      if let (Some(l), Some(r)) = (left_prompt.interact(), right_prompt.interact()) {
        let left = Note::from(self.directory.path.join(l.first().unwrap()));

        let right = Note::from(self.directory.path.join(r.first().unwrap()));

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

        if !left.has_link(&right.id.to_string()) && !right.has_link(&left.id.to_string()) {
          println!("The two notes are already unlinked.");
          return Ok(());
        }

        if left.has_link(&right.id.to_string()) {
          left.remove_link(&right.id.to_string())?;
        }

        if right.has_link(&left.id.to_string()) {
          right.remove_link(&left.id.to_string())?;
        }

        println!("{} <-X-> {}", left.id, right.id);
      } else {
        println!("You must choose two notes in order to link them together.");
        return Ok(());
      }
    } else {
      println!("Both notes must exist in order to be linked.");
      return Ok(());
    }
    Ok(())
  }

  pub fn tag(&self, _name: &str, _tag: &str) -> Result<(), Error> {
    Ok(())
  }

  pub fn remove_tag(&self, _name: &str, _tag: &str) -> Result<(), Error> {
    Ok(())
  }
}
