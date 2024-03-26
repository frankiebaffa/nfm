# No-Flavor Markdown

> _I got no flavor, it's what I deserve, really._  
> - Michael, _The Good Place_

- - -

No-Flavor is yet another attempt at simplifying, optimizing, and tightening up
markdown syntax.

## Block-Level Elements

### Headers

```markdown
# One
## Two
### Three
#### Four
##### Five
###### Six
```

Headers remain unchanged for the most part. Only
[atx](http://aaronsw.com/2002/atx/intro) style headers are supported.
[Setext](https://docutils.sourceforge.io/mirror/setext.html) style headers have
been ditched.

## Horizontal Rules

```markdown
- - -
```

Horizontal rules are defined using three space-delimited hyphens followed by a
line-break. Three space-delimited underscore support has been dropped.

## Lists

```markdown
- This
- is
    - an unordered
- list.
```

```markdown
0. This
0. is
    0. an ordered
0. list.
```

```markdown
- This
- is
    0. a mixed
- list.
```

Unordered lists are defined by a line beginning with a hyphen while not in a
paragraph or a line beginning with spaces followed by a hyphen when attempting
to nest.

Ordered lists are defined within the same parameters, only with a zero folled by
a period. Other numbers are not supported.

Mixed lists are allowed. Although alternating between two different types within
the same level will result in the first-defined level being used.

## Code Blocks

```markdown
    let mut x = 0;
    let mut y = 1;
    std::mem::swap(&mut x, &mut y);
    y += 1;
    assert_eq!(x, y);
```

Generic code blocks are defined by beginning a line with four spaces.

## Code Fences

```markdown
\`\`\`rust
let mut x = 0;
let mut y = 1;
std::mem::swap(&mut x, &mut y);
y += 1;
assert_eq!(x, y);
\`\`\`
```

Code fences are defined by beginning a line with three backticks. A language
name is supported and optional.

## Blockquotes

```markdown
> I got no flavor, it's what I deserve, really.
```

Blockquotes are defined by beginning a line with a greater-than sign.

## Tables

Tables are where some may think this gets a little ugly. Tables are entirely
changed, but their functionality is increased.

```markdown
|=^ Left aligned col header |= Center aligned header |=$ Right aligned header
|-^ Left aligned row header |  Center aligned cell   |$  Right aligned cell
|^  Left aligned cell       |  Center aligned cell   |$  Right aligned cell
```

Tables no longer have a trailing delimiter for the final cell in the row and
the properties of _each_ cell is defined by leading flags. The order of the
flags are as follows:

- Cell type
    - `=` column header
    - `-` row header
    - `_` standard cell _(default)_
- Alignment
    - `^` left
    - `$` right
    - `_` center _(default)_
- V-Alignment
    - `t` top
    - `m` middle
    - `b` bottom
    - `_` baseline _(default)_
- Col-span
    - _n_ the number of cells
    - _default_ is 1
- Row-span
    - _,n_ the number of cells
    - _default_ is 1

Any of these flags can be left out entirely as none of them collide. The `_`
character can be used to signify the default value for alignment purposes.

The flag design allows for much more robust table definitions as each cell can
be defined with individual formatting.

```markdown
|=^m_,3 Left aligned, 3-row |=$_3,_                                           Right aligned 3-column
                            |-^m_,2 Left aligned |=^__,_ Left aligned |_$__,_ Right aligned
                                                 |=^__,_ Left aligned |_$__,_ Right aligned
```

## Paragraphs

```markdown
This is a paragraph.
```

Paragraphs are everything else.

# Inline Elements

## Strong

```markdown
This paragraph has some **strong** words.
```

Strong text is defined by encapsulating the desired characters with two
astericks.

## Emphasis

```markdown
It can be this _or_ this.
```

Emphasized text is defined by encapsulating the desired characters with a single
underscore. Support for a single astericks was dropped.

## Deleted

```markdown
~~This is how you do it.~~ I changed my mind.
```

Deleted text is defined by encapsulating the desired characters with two tildes.

## Inserted

```markdown
What the ~~hell~~ ++heck++.
```

Inserted text is defined by encapsulating the desired characters with two plus
signs.

## Marked

```markdown
Just gonna highlight ==this== real quick.
```

Marked text is defined by encapsulating the desired characters with two equals
signs.

## Superscript

```markdown
E=mc^2^
```

Superscript is defined by encapsulating the desired characters with a carrot.

## Anchors

```markdown
# <desired-id>Id'ed Header
```

Anchors are defined by encapsulating the desired ID with a less-than sign and
a greater-than sign. The above anchor would be parsed into:
`<a id="desired-id"></a>`.

## Checkboxes

```markdown
[ ] Incomplete item

[x] Complete item
```

Checkboxes are inline elements, not special list types. An unchecked checkbox
is defined as an open square bracket followed by a space followed by a closed
square bracket. Checked checkboxes are defined as an open square bracked
followed by an _x_ followed by a closed square bracket.

## Links

```markdown
Please click [here](http://frankiebaffa.com/projects/nfm.html).
```

Links are defined in two parts. The link text: desired text encapsulated with
open/closed square brackets and the link url: desired url encapsulated with
open/closed parentheses.

## Images

```markdown
![an image](http://frankiebaffa.com/favicon-16x16.png)
```

Images are identical to links, only with a leading exclamation point. The
text defines the `alt` text property.

## Inline Code

```markdown
Here is `let x: usize = 0;` some code.
```

Inline code can be defined by encapsulating the desired code with backticks.

# Tricks

## Footnotes

Using anchors, links, and superscript combined; you can mimic footnotes found
in more bloated markdown parsers:

```markdown
This is a fact^[1](#fact)^.

<fact>1: I made it up.
```
