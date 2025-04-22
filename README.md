# Similar Image Detector

This is a Rust command-line tool to find similar images within a specified folder and its subfolders. It uses perceptual hashing algorithms to compare images and can perform different actions (print, move, copy, or delete similar images) based on user choice.

## Features

-   Recursively finds images in folders (supports jpg, jpeg, png, bmp, webp, tiff, tif, ico, avif [if avif feature is enabled]).
-   Calculates perceptual hashes of images using the `image-hasher` crate.
-   Computes hashes in parallel for speed.
-   Groups similar images using the Disjoint Set Union (DSU) algorithm.
-   Provides different actions to handle groups of similar images:
    -   `print`: Prints the paths of similar images.
    -   `move`: Moves similar images to a specified target folder (one subfolder per group).
    -   `copy`: Copies similar images to a specified target folder (one subfolder per group).
    -   `delete`: Deletes all but the largest file (by size) in each group of similar images.
-   Configurable similarity threshold.
-   Displays a progress bar.

## Building

Ensure you have Rust and Cargo installed.

```bash
git clone <repository-url>
cd similarImgDetect
cargo build --release
```

**To support AVIF images, you need to follow these steps:**

1.  Install `pkg-config` and `vcpkg` through [chocolatey](https://chocolatey.org/) or [scoop](https://scoop.sh/)

    ```powershell
    choco install pkgconfiglite
    choco install vcpkg
    ```

    or

    ```powershell
    scoop install pkg-config
    scoop install vcpkg
    ```

2.  Install `dav1d`

    ```powershell
    vcpkg install dav1d:x64-windows
    ```

3.  Add to the `PKG_CONFIG_PATH` environment variable the path `$VCPKG_INSTALLATION_ROOT\installed\x64-windows\lib\pkgconfig`

4.  Build code

```bash
    git clone <repository-url>
    cd similarImgDetect
    cargo build --release --features avif
```

To speed up the computation, you can build your packages only in `Release` mode
adding the `set(VCPKG_BUILD_TYPE release)` line to the
`$VCPKG_INSTALLATION_ROOT\triplets\x64-windows.cmake` file.

Building for Windows x86 is the same, just replace `x64` with `x86` in the
steps above.

## Usage

```bash
./target/release/similarImgDetect --path <image-folder> [options]
```

### Options

-   `-p, --path <path>`: Path to the folder containing images (required).
-   `-s, --similarity <similarity>`: Similarity threshold for image comparison (float between 0-1, default: 0.9). Higher values mean more similarity is required.
-   `-a, --action <action>`: Action to perform after finding similar images (default: `print`).
    -   `print`: Print the paths of similar images.
    -   `move`: Move similar images to the folder specified by `--target-path`.
    -   `copy`: Copy similar images to the folder specified by `--target-path`.
    -   `delete`: Delete all but the largest file in each group of similar images.
-   `-t, --target-path <target_path>`: Path to the target folder for moving/copying similar images when `action` is `move` or `copy`.

## Examples

1.  **Print** groups of images with similarity >= 0.95 in the `/path/to/images` folder:

    ```bash
    ./target/release/similarImgDetect -p /path/to/images -s 0.95 -a print
    ```

    Or (since `print` is the default action):

    ```bash
    ./target/release/similarImgDetect -p /path/to/images -s 0.95
    ```

2.  **Move** images with similarity >= 0.9 from `/path/to/images` to `/path/to/similar_images` (one subfolder per group):

    ```bash
    ./target/release/similarImgDetect -p /path/to/images -a move -t /path/to/similar_images
    ```

3.  **Copy** images with similarity >= 0.85 from `/path/to/images` to `/path/to/duplicates`:

    ```bash
    ./target/release/similarImgDetect -p /path/to/images -s 0.85 -a copy -t /path/to/duplicates
    ```

4.  **Delete** similar images with similarity >= 0.98 in `/path/to/images` (keeping the largest file in each group):
    ```bash
    ./target/release/similarImgDetect -p /path/to/images -s 0.98 -a delete
    ```

## Dependencies

-   `clap`: For command-line argument parsing.
-   `image-hasher`: For generating image hashes.
-   `image`: For image loading and processing.
-   `rayon`: For parallel processing.
-   `indicatif`: For displaying progress bars.

## License

This project is licensed under the MIT License.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```

```

```

```
