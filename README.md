## Anvl — v0.0.1

**Anvl** is a command-line tool built to reduce **setup friction** and **mental load** when working on **C projects**, especially in **strict or educational environments** like **Epitech** or **42**.

It focuses on three core ideas:

* **Fast project initialization**
* **Safe, explicit module reuse**
* **Keeping your build system honest**

> **No magic. No code generation.**
> Just automation, structure, and transparency.

## Why Anvl exists

If you’ve worked on C projects at school for any amount of time, you’ve probably hit the same walls over and over:

* **Short deadlines, stacked projects**
  With 2–4 weeks per project (often several at once), wasting hours recreating the same structure is not an option.

* **Library reuse is useful… until it isn’t**
  Writing your own library is great, but reusing it blindly isn’t.
  Most projects only need *a subset* of functions, and every reused feature still needs to comply with the subject’s authorized functions.

* **Makefiles aren’t hard, just exhausting**
  Editing a Makefile isn’t complicated, but it *is* constant mental overhead.
  The best solution is not having to think about it at all.

Anvl exists to remove these frictions **without hiding what’s really happening**.

## What Anvl is *not*

Anvl is intentionally limited. It does **not** try to be:

* ❌ A build system
* ❌ A compile-time dependency resolver
* ❌ A framework
* ❌ A package manager that hides source code

Anvl works **with** your existing workflow, not **against** it.

## Typical workflow

### Initialize a project

```sh
anvl init bin my_project --push
```

* Fetches the `bin` template from your Anvl repository
* Renames the binary, includes, and structure to `my_project`

### List available modules

```sh
anvl list
```

* Lists every module available in your **local Anvl repository cache**

### Install modules

```sh
anvl install io str vec --push
```

* Installs `io`, `str`, and `vec`
* Automatically resolves and installs their dependencies
* Copies the code directly into the project

### Create files from templates

```sh
anvl create c:cli modules/cli/cli --push
```

* Creates `cli.c` in `src/modules/cli` using the `c:cli` template
* Creates a matching test file in `tests/modules/cli` using a project-specific test template if available, falls back to the global test template otherwise
* Automatically adds both files to the Makefile

> [!NOTE]
>
> Some commands accept the `--push` flag.
> When enabled, Anvl automatically commits and pushes the affected changes to your repository.

> [!NOTE]
>
> This is only the **visible tip of the iceberg**.
