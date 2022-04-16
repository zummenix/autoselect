# autoselect

Scrapes car infos from https://autoselect.ru/

> NOTE: Scrapper doesn't work because autoselect.ru had changed their website some time ago.
> Right now I don't have time and motivation to update it.

## Usage

Make sure you have rust compiler installed and then run `cargo install --release` in the directory.
- `autoselect` CLI tool, prints the result
- `./compare` script runs `autoselect`, saves the result in a file and shows the diff using neovim

## Configuration

Head to the site, choose your parameters and replace `BASE_URL` in `src/main.rs` with your url.
After that you need to compile the project again (`cargo install --release`).

> Note: make sure your filter is narrow enough to not have a lot of pages, since the tool will
> perform several requests one after another and it may take a lot of time.

