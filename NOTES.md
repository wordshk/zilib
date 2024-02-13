# Notes

- There's apparently no standard way to include data files as part of a
  library, see eg. https://github.com/rust-lang/rfcs/pull/2376 . include_str!
  and include_bytes! isn't efficient. We might have to let the user deal with
  the logistics of interpretting the unihan database. We can provide some
  tooling though.
