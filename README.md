# 「ゼロからのOS自作入門」をmac+Rustで

https://www.amazon.co.jp/ゼロからのOS自作入門-内田-公太/dp/4839975868

環境:

- OS: macOS 14.5
- CPU: Apple M2 Pro
- Rust: rustc 1.81.0-nightly (9057c3ffe 2024-07-19)
- `~/workspace/mikanos-build`に公式のリポジトリをクローン

  https://github.com/uchan-nos/mikanos-build

## 本からの変更点

### 1.4 エミュレータでのやり方

- QEMUはbrewでインストールした

    ```shell
    $ brew install qemu
    $ brew info qemu
    ==> qemu: stable 9.0.2
    ```

- mkfs.fatコマンドがなかったのでbrewでインストールした

    ```shell
    $ brew install dosfstools
    $ brew info dosfstools
    ==> dosfstools: stable 4.2
    ```

- OS標準のmountコマンドがうまく動かず、hdiutilコマンドを使った

    ```shell
    # マウント
    hdiutil attach -imagekey diskimage-class=CRawDiskImage -mountpoint ./mnt disk.img
    
    # アンマウント
    hdiutil declare ./mnt
    ```

    <details>
    <summary>OS標準のmountコマンドのエラー</summary>

    - 本で紹介されているコマンドの場合、以下のエラー

        ```shell
        $ sudo mount -o loop disk.img ./mnt
        mount: you must specify the filesystem type
        ```

    - `-t` オプションでファイルシステムを指定したが、対応するドライバ(?)がないらしくエラー

        ```shell
        $ sudo mount -t vfat -o loop disk.img ./mnt
        mount: exec /Library/Filesystems/vfat.fs/Contents/Resources/mount_vfat: No such file or directory
        ```

    - `vfat`の代わりに`msdos`
      を指定するとよいという[情報](https://apple.stackexchange.com/questions/260374/mount-fat32-partition-external-drive-on-mac
      )を見つけたが、このドライバはループバックオプションに対応してない様子。

        ```shell
        $ sudo mount -t msdos -o loop disk.img ./mnt
        mount_msdos: -o loop: option not supported
        ```

    </details>