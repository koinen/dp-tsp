# dp-tsp
## Pemecah Permasalahan TSP Menggunakan Dynamic Programming
> Dibuat untuk IF2211 - Strategi Algoritma

## Deskripsi

Program ini menyelesaikan permasalahan Traveling Salesman Problem (TSP) menggunakan strategi Dynamic Programming (DP). 

TSP adalah permasalahan optimasi untuk mencari rute terpendek yang mengunjungi setiap kota tepat satu kali dan kembali ke kota asal. 

## Cara Kerja

Dapat diamati bahwa dalam TSP, terdapat pengulangan terhadap beberapa upa-persoalan, seperti rute ` 1 -> 2 -> 3 -> 4 ` dan ` 1 -> 2 -> 4 -> 3` memiliki pengulangan pada upa-rute ` 1 -> 2 `.

Oleh karena itu, dapat dilakukan memoisasi (Top-Down DP) untuk menghilangkan perhitungan redundan dengan menyimpan nilai tersebut dalam memori.

## Implementasi

Program ini diimplementasikan dalam bahasa pemrograman Rust.

Untuk memoisasinya, disimpan dalam array 2D dengan dimensi rute yang sudah diambil (simpul-simpul yang sudah dikunjungi) dan simpul terakhir yang dikunjungi dari rute tersebut. Untuk menyimpan rute yang sudah diambil digunakan suatu mask 32-bit (integer) untuk menyimpan simpul-simpul yang sudah dikunjungi, berdasarkan bit-nya, seperti simpul ke-0 dan ke-2 sudah dikunjungi, maka ` ..0101 `.

Sehingga, jika memoisasi untuk rute dari simpul tersebut sudah ada, tidak perlu dihitung lagi dan langsung mengembalikan dengan nilai memoisasinya.

## Prasyarat

Sebelum menjalankan program ini, pastikan telah menginstal:

- [Rust](https://www.rust-lang.org/tools/install)

Pastikan juga terminal/command prompt sudah dapat menjalankan perintah `cargo`.

## Fitur

- Input jumlah kota dan matriks jarak antar kota
- Implementasi algoritma DP untuk TSP
- Output rute optimal dan total jarak minimum

## Cara Menjalankan

### Kompilasi Manual
1. Clone repositori ini:
    ```bash
    git clone https://github.com/koinen/dp-tsp.git
    cd dp-tsp
    ```
2. Jalankan program:
    ```bash
    cargo run -- --file-path [file .txt dalam test]
    ```

### Menggunakan Release 
1. Unzip file .zip release
2. Jalankan program
    ```bash
    .path/to/release/dp-tsp.exe --file-path [file .txt dalam test]
    ```

## Author

- David Bakti Lodianto - 13523083