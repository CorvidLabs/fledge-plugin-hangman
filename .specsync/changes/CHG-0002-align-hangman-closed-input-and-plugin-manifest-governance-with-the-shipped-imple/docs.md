---
change: CHG-0002-align-hangman-closed-input-and-plugin-manifest-governance-with-the-shipped-imple
artifact: docs
---

# Docs

No user command syntax or runtime behavior changes. The canonical contract now explicitly states that EOF is
unsupported as a graceful termination mechanism in the shipped interactive loop. The plugin manifest joins the
governed file set because it defines the public `hangman` command and binary binding.
