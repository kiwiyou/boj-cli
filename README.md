<div align="center">
    <h1><code>boj-cli</code></h1>
    <p>여러모로 귀찮은 사람들을 위한 BOJ 명령줄 인터페이스</p>
</div>

설치
---

`cargo`를 이용하여 설치할 수 있습니다.

```bash
gh repo clone kiwiyou/boj-cli
# 또는
git clone https://kiwiyou/boj-cli.git
cd boj-cli

cargo install --path .
```

사용법
---

- `boj new <id>`
  - `<id>`번 문제를 위한 디렉토리를 `./<id>` 위치에 생성합니다.

- `boj new <id> --title <title>`:
  - `<id>`번 문제를 위한 디렉토리를 `./<id>-<title>` 위치에 생성합니다.
  - `<title>`은 영문 kebab-case가 권장됩니다.

디렉토리 구조
---

- `./template`
  - 문제 디렉토리를 생성하기 위한 템플릿 파일을 저장하는 곳입니다.
  - `Tera` 템플릿 엔진을 사용합니다.

- `./template/note.md`
  - 문제 풀이 노트의 템플릿입니다.

- `./template/solution.rs`
  - 문제 풀이 코드의 템플릿입니다.

- `./<id>[-<title>]`
  - `<id>`번 문제를 위한 디렉토리입니다.

- `./<id>[-<title>]/meta.json`
  - 문제 풀이 정보를 담은 파일입니다.

- `./<id>[-<title>]/note.json`
  - 문제 풀이 노트입니다.

- `./<id>[-<title>]/note.json`
  - 문제 풀이 코드입니다.
