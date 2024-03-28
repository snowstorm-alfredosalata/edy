<!-- omit in toc -->
# Contributing to Edy
Thanks for your interest in Edy.

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. 

<!-- omit in toc -->
## Table of Contents

- [Submitting Questions](#submitting-questions)
- [Contributing](#contributing)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Contributing to the development](#contributing-to-the-development)
    - [Coding style](#coding-style)
    - [Commit Messages](#commit-messages)

## Submitting Questions

> If you want to ask a question, I assume that you have read the available [Documentation](https://docs.rs/edy/).

Before you ask a question, it is best to search for existing [Issues](https://github.com/snowstorm-alfredosalata/edy/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/snowstorm-alfredosalata/edy/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

I will then take care of the issue as soon as possible. Please understand that this project is maintained by a solo developer, so I might not be able to answer on a daily

## Contributing

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, I ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://docs.rs/edy/). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/snowstorm-alfredosalata/edy/issues?q=label%3Abug).


<!-- omit in toc -->
#### How to file a bug report

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to as@salata.ovh.

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/snowstorm-alfredosalata/edy/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.

Also be sure to include this information:
- Stack trace (Traceback)
- OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
- Rust compiler version

Once it's filed:

- I will review and label issue accordingly.
- If there are no reproduction steps or no obvious way to reproduce the issue, I will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- Once I am able to address the issue I will mark it as `needs-fix`, as well as possibly other tags (such as `critical`). If I am not able to tackle the issue immediately, it will be left to be [implemented by someone](#your-first-code-contribution).


### Suggesting Enhancements

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://docs.rs/edy/) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/snowstorm-alfredosalata/edy/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Be sure that idea fits with the scope and aims of the project.

<!-- omit in toc -->
#### How to submit an Enhancement Suggestion

Enhancement suggestions are tracked as [GitHub issues](https://github.com/snowstorm-alfredosalata/edy/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- **Explain why this enhancement would be useful** to most Edy users. You may also want to point out the other projects that solved it better and which could serve as inspiration.


### Contributing to the development
I currently do not accept code contributions. I will do so, when I get around to setting-up the repository accordingly.


#### Coding style
Coding style should conform to Rust conventions. Additional, Clean Code and Single Responsibility principles should generally be applied.

#### Commit Messages
Commit messages adhere to [Conventional Commits](https://conventionalcommits.org). You should also propose your changelog 
