from dataclasses import dataclass


@dataclass
class Section:
    title: str
    body: list


@dataclass
class Category:
    title: str
    sections: list


@dataclass
class Result:
    categories: list
    msgs: list


TEMPLATE = """
# Full List of Messages

xplr messages categorized based on their purpose.

## Categories

{categories}

{msgs}

## Also See:

- [Message](message.md)
"""


def gen_messages():
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
            sec = Section(title=None, body=[])
            cat = Category(title=line, sections=[sec])
            res.append(cat)
            continue

        if line.startswith("/// "):
            line = line.lstrip("/// ").strip()
            res[-1].sections[-1].body.append(line)
            continue

        elif not line.strip() or line.strip() == "///":
            res[-1].sections[-1].body.append("")

        elif "," in line:
            line = line.split(",")[0].split("(")[0]
            res[-1].sections[-1].title = line

            sec = Section(title=None, body=[])
            res[-1].sections.append(sec)
            continue

    result = Result(categories=[], msgs=[])

    for cat in res:
        slug = cat.title.lower().replace(" ", "-")
        result.categories.append(f"- [{cat.title}](#{slug})")
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

    return result


def main():
    res = gen_messages()
    doc = TEMPLATE.format(
        categories="\n".join(res.categories), msgs="\n".join(res.msgs)
    )

    print(doc)
    with open("./docs/en/src/messages.md", "w") as f:
        print(doc, file=f)


if __name__ == "__main__":
    main()
