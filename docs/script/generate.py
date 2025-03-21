"""Generate docs from comments."""

import os
from dataclasses import dataclass
from typing import List

# Messages --------------------------------------------------------------------

MESSAGES_DOC_TEMPLATE = """
# Full List of Messages

xplr [messages][1] categorized based on their purpose.

## Categories

{categories}

{msgs}

## Also See:

- [Message][1]

[1]: message.md
""".strip()

CONFIGURATION_DOC_TEMPLATE = """
# Configuration

{doc}

""".strip()


@dataclass
class MsgSection:
    title: str | None
    body: List[str]


@dataclass
class MsgCategory:
    title: str
    sections: List[MsgSection]


@dataclass
class MsgResult:
    categories: List[MsgCategory]
    msgs: List[str]


def gen_messages():
    """Generate messages.md"""

    path = "./src/msg/in_/external.rs"
    res = []
    reading = False

    with open(path) as f:
        lines = iter(f.read().splitlines())

    for line in lines:
        line = line.strip()

        if line.startswith("pub enum ExternalMsg {"):
            reading = True
            continue

        if not reading:
            continue

        if line == "}":
            break

        if line.startswith("/// ### "):
            line = line.lstrip("/// ### ").rstrip("-").strip()
            sec = MsgSection(title=None, body=[])
            cat = MsgCategory(title=line, sections=[sec])
            res.append(cat)
            continue

        if line.startswith("/// "):
            line = line.lstrip("/// ").strip()
            res[-1].sections[-1].body.append(line)
            continue

        if not line or line == "///":
            res[-1].sections[-1].body.append("")
            continue

        if line.endswith(","):
            line = line.split(",")[0].split("(")[0]
            res[-1].sections[-1].title = line

            sec = MsgSection(title=None, body=[])
            res[-1].sections.append(sec)
            continue

    result = MsgResult(categories=[], msgs=[])

    for cat in res:
        slug = cat.title.lower().replace(" ", "-")
        result.categories.append(
            MsgCategory(title=f"- [{cat.title}](#{slug})", sections=[])
        )
        result.msgs.append(f"### {cat.title}")
        result.msgs.append("")

        for sec in cat.sections:
            if not sec.title:
                continue

            result.msgs.append(f"#### {sec.title}")
            result.msgs.append("")
            for line in sec.body:
                result.msgs.append(f"{line}")
            result.msgs.append("")

    messages = MESSAGES_DOC_TEMPLATE.format(
        categories="\n".join(c.title for c in result.categories),
        msgs="\n".join(result.msgs),
    )

    print(messages)
    with open("./docs/en/src/messages.md", "w") as f:
        print(messages, file=f)


# Configuration ---------------------------------------------------------------


def gen_configuration():
    """Generate the following docs.

    - configuration.md
    - general-config.md
    - node_types.md
    - layouts.md
    - modes.md
    - modes.md
    """

    path = "./src/init.lua"

    configuration = [[]]
    general = [[]]
    node_types = [[]]
    layouts = [[]]
    modes = [[]]

    with open(path) as f:
        lines = iter(f.read().splitlines())

    reading = None

    for line in lines:
        if line.startswith("---"):
            continue

        if (
            line.startswith("-- # Configuration ")
            or line.startswith("-- ## Config ")
            or line.startswith("-- ## Function ")
            or line.startswith("-- ## On Load ")
        ):
            reading = configuration

        if line.startswith("-- ### General Configuration "):
            reading = general

        if line.startswith("-- ### Node Types "):
            reading = node_types

        if line.startswith("-- ### Layouts "):
            reading = layouts

        if line.startswith("-- ### Modes "):
            reading = modes

        if not reading:
            continue

        if line.startswith("-- ") or line == "--":
            if line.startswith("-- #") and line.endswith("--"):
                line = "\n{0}\n".format(line.rstrip("-"))

            reading[-1].append(line[3:])
            continue

        if line.startswith("xplr.") and reading[-1]:
            reading[-1].insert(0, "\n#### {0}\n".format(line.split()[0]))
            continue

        if not line.strip() and reading[-1]:
            reading.append([])
            continue

    with open("./docs/en/src/configuration.md", "w") as f:
        doc = "\n".join(["\n".join(c) for c in configuration])
        print(doc)
        print(doc, file=f)

    with open("./docs/en/src/general-config.md", "w") as f:
        doc = "\n".join(["\n".join(c) for c in general])
        print(doc)
        print(doc, file=f)

    with open("./docs/en/src/node_types.md", "w") as f:
        doc = "\n".join(["\n".join(c) for c in node_types])
        print(doc)
        print(doc, file=f)

    with open("./docs/en/src/layouts.md", "w") as f:
        doc = "\n".join(["\n".join(c) for c in layouts])
        print(doc)
        print(doc, file=f)

    with open("./docs/en/src/modes.md", "w") as f:
        doc = "\n".join(["\n".join(c) for c in modes])
        print(doc)
        print(doc, file=f)


# xplr.util -------------------------------------------------------------------


@dataclass
class Function:
    doc: List[str]
    name: str


def gen_xplr_util():

    path = "./src/lua/util.rs"

    functions: List[Function] = []

    with open(path) as f:
        lines = iter(f.read().splitlines())

    reading = None

    for line in lines:
        if line.startswith("///"):
            if reading:
                reading.doc.append(line[4:])
            else:
                reading = Function(doc=[line[4:]], name="")

        if line.startswith("pub fn") and reading:
            reading.name = "\n### xplr.util." + line.split("(", 1)[0].split()[-1] + "\n"
            functions.append(reading)
            reading = None
            continue

    with open("./docs/en/src/xplr.util.md", "w") as f:
        for function in functions:
            print(function.name)
            print(function.name, file=f)

            print("\n".join(function.doc))
            print("\n".join(function.doc), file=f)

        if reading:
            print("\n".join(reading.doc), file=f)


def format_docs():
    os.system("prettier --write docs/en/src")


def main():
    gen_messages()
    gen_configuration()
    gen_xplr_util()
    format_docs()


if __name__ == "__main__":
    main()
