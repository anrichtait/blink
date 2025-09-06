# blink

**Fast, practical fuzzy search.**
Search filenames, content, projects, media and documents.

Built as an answer to all the sub-par tmux and bash scripts flying around. Also a hobby project.


---

## Why blink

* Instant file discovery and string search
* Designed to be as small as possible
* Focused UX for everyday work.
* Sensible defaults

---

## Dependencies
**AUR Packages:**
* `fd` (filename search)
* `rg` (content search)
* `fzf`(picker)
* `nvr`/`nvim` (editor integration)
* `mpv`, `zathura` or your preferred media/document viewers
**Crates**
* clap
* 

---

## Commands (one-line behaviors)

* `gf [query]` — fuzzy filename search from `~` and `/mnt`. No query → interactive list.
* `gs [pattern]` — system-wide content search (ripgrep).
* `pf [query]` — find project directories (`.git`, `Cargo.toml`, `package.json`).
* `ps [pattern]` — content search scoped to nearest project root.
* `ms [query]` — media (images/audio/video). Previews where available. Multi-select supported.
* `df [query]` — document search (`.pdf`, `.docx`) and open in configured viewer.
* `none` or no command — system-wide search including `/mnt` and specified folders.

Flags:
A tool like this should focus on being as frictionless as possible. Flags are a convenient way
to quickly change program defaults but add extra usage overhead for users. For that reason all
tool configuration is done in the source or in the blink.toml config file.

---

## Config (example)

```toml
# ./.config/blink/blink.toml
fav_dirs = ["~/Projects", "/mnt", "$HOME/work"]
exclude = [".git", "node_modules", "target", "/proc", "/sys"]
editor = "nvim"                      # fallback: "bat"
preview = "true" # or false
project_markers = [".git","Cargo.toml","package.json"]
max_depth = 0
max_preview_size = 5    #in mb
```

Keep config focused: start\_dirs, exclude, editor, media\_player, document\_viewer, preview\_cmds, project\_markers.

---

## Integration rules

* If `$NVIM_LISTEN_ADDRESS` + `nvr` present → `nvr --remote-tab-silent <file>`.
* Else → spawn `nvim <file>`.
* If `emacsclient` daemon available → `emacsclient -n`.
* Multi-select passes all paths to configured app (e.g., `mpv file1 file2`).

---

## Performance rules

* Use `fd` for traversal and `rg` for content. They skip irrelevant files and are fast on code trees.
* Default exclude list: `.git`, `node_modules`, `target`, `build`, `dist`, `__pycache__`, `.DS_Store`, `/proc`, `/sys`, `/dev`.
* Default: limit system-wide depth. Use `ps` for deep project search.
* Don’t preview files larger than `size_preview_limit_mb`.

---

## Roadmap (concise)

1. Docs & config (this README).
2. Glue script: `fd` → `fzf`/`sk` → opener (shell or tiny Rust with pico-args).
3. UX polish: preview mapping, multi-select rules, robust project detection.
4. Optional: single-binary rewrite (embed picker) and package for AUR.

---

