# Getting Started with lq

**LangQuest** is the interactive runner you use for every exercise in this
course. It shows the exercise text, watches the exercise file, and - every time
you **save** - recompiles it and re-runs the unit tests automatically. You never
run the tests by hand: just edit `main.rs`, save, and watch the **Output** page.

## The Two Views

LangQuest has two screens:

- **Overview** - the tree of all chapters and exercises, with your progress.
- **Exercise view** - a single exercise, split into pages.

An exercise is made of several **pages** you move between with the arrow keys:

| Page         | What it shows                                                        |
|--------------|---------------------------------------------------------------------|
| **Theory**   | The concept you need (this page).                                    |
| **Task**     | What you have to implement.                                          |
| **Debug**    | Compiler messages and diagnostics for your current code.            |
| **Output**   | The test results and your score (tests passed / total).            |
| **Solution** | The reference solution - **locked** until you pass or reveal it.    |

## Shortcuts

### Overview page

| Key         | Action                                 |
|-------------|----------------------------------------|
| `↑` / `↓`   | Navigate the exercise list             |
| `Enter`     | Open the selected exercise             |
| `z`         | Collapse / expand all exercise folders |
| `a`         | About                                  |
| `m`         | Show / hide the menu                    |
| `q`         | Quit LangQuest                         |

### Inside an exercise

| Key         | Action                                                        |
|-------------|--------------------------------------------------------------|
| `←` / `→`   | Jump between pages (Theory, Task, Debug, Output, Solution)    |
| `↑` / `↓`   | Scroll within the current page                               |
| `j` / `k`   | Jump to the previous / next exercise                          |
| `e`         | Open the current exercise file in your editor                |
| `h`         | Reveal the next hint (see below)                             |
| `o`         | Go back to the Overview page                                  |
| `a`         | About                                                        |
| `m`         | Show / hide the menu                                          |
| `q`         | Quit LangQuest                                               |

## Hints and the Solution

Stuck? Press `h` to reveal hints **one at a time**, from a gentle nudge toward a
more concrete pointer. After the **last** hint, pressing `h` once more unlocks
and shows the full **Solution** page.

**Hint reveals and solution reveals are recorded in your progress file.** While
an exercise is still unsolved, LangQuest stores how many hints you revealed and
whether you looked at the solution. This progress is integrity-protected, so it
cannot be quietly edited away - your instructor can see it.

This is not meant to punish you: hints and solutions are there to help you learn.
But try the exercise yourself first - reach for a hint only when you are truly
stuck, and open the solution only after you have given it a genuine attempt.
Once you have **passed** an exercise, revealing hints or the solution to review
it is free and no longer counted.

Ready? Move to the **Task** page (`→`) and fix your first program.
