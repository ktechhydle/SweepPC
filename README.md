<img src="resources/icons/sweep_pc_icon.svg" width="125">

# SweepPC
It's time to take control of your Windows computer; keep it clean
and running smooth with a command line tool.

```
C:/Users/JohnDoe> sweeppc run
⏩ Running default cleanup
🕵️ Scanning for large and old files (>100 MB, older than a year)
🗑️ Scanning for temporary files
🔎 Searching 'C:/Users/JohnDoe/Desktop'
🔎 Searching 'C:/Users/JohnDoe/Documents'
🔎 Searching 'C:/Users/JohnDoe/Downloads'
🔎 Searching 'C:/Users/JohnDoe/Videos'
🔎 Searching 'C:/Users/JohnDoe/Pictures'
🔎 Searching 'C:/Users/JohnDoe/Music'
🔎 Searching 'C:/Users/JohnDoe/AppData/Local/Temp/'
🎉 SweepPC found 1 result
📁 File 'C:/Users/JohnDoe/AppData/Local/Temp/a_very_large_dll.dll'

🤔 Would you like to delete the found files? (y/n): y
✅ Deleted found files
```

## Features

**Scan Your Computer 🔎**

SweepPC scans your system for _large_ and _outdated files_ with one simple command:

```
sweeppc run
```

**Target Specific Directories 🎯**

Only want to clean up a specific folder? Use the `runtarget` command:

```
sweeppc runtarget /path/to/my/dir
```

**Clean Temp Files 📂**

Sweep out all those leftover temporary files from apps and Windows:

```
sweeppc cleantemp
```

# Why SweepPC?
- ⚡ Fast and lightweight
- 🧠 Smart file detection (by size and age)
- 🧼 Cleans intelligently, no guesswork
- 🤖 Fully terminal-based, perfect for power users
- 💻 Built with Rust, made for Windows

# Free & Open Source
SweepPC is proudly open-source and licensed under the GPL. No nags. No premium. No nonsense.
Just you, your terminal, and a sparkling-clean machine👍

**Contributions welcome! Fork it, star it, and sweep on.**
