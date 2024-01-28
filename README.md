# txt_to_md

This repository `txt_to_md` is the convert CLI from txt files to markdown file.

## Get started

- Install

```
cargo install txt_to_md
```

- Prepare text.txt

```
## a
### bb

ccc
ddd
eee
fff

ggg
hhh
iii
jjj
```

- Run

```
txt_to_md -i text.txt -o output.md
```

- output.md

```md
## a
### bb

- ccc
  - ddd
  - eee
  - fff
- ggg
  - hhh
  - iii
  - jjj
```

## Option

- See by below command

```
txt_to_md -h
```

- Output example if is_plane_text option uses

```md

## a
### bb


- ccc

ddd
eee
fff

- ggg

hhh
iii
jjj
```

## History

- For next release
  - Updated README
- 0.1.1
  - Updated README
- 0.1.0
  - Release first prototype

