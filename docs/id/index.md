---
# https://vitepress.dev/reference/default-theme-home-page
layout: home
lang: id-ID

head:
  - - meta
    - name: description
      content: "LightVM adalah mesin virtual berbasis kapabilitas yang dirancang untuk eksekusi bytecode yang aman, prediktif, dan efisien dengan penggunaan memori minimal."
  - - meta
    - name: keywords
      content: "LightVM, eksekusi bytecode, virtual machine, keamanan, pemrograman sistem, berbasis kapabilitas"
  - - meta
    - property: og:title
      content: "LightVM | VM Bytecode yang Aman & Teroptimasi"
  - - meta
    - property: og:description
      content: "Eksekusi minimalis untuk keamanan maksimal. Jelajahi dokumentasi LightVM."
  - - meta
    - property: og:url
      content: "https://lightvm.vercel.app/id/"

title: Dokumentasi Utama
description: Mesin virtual berbasis kapabilitas yang dirancang untuk eksekusi bytecode yang aman, prediktif, dan efisien.

hero:
  name: "LightVM"
  text: "Mesin virtual berbasis kemampuan yang dirancang untuk eksekusi bytecode yang aman, dapat diprediksi, dan optimal."
  tagline: Eksekusi Minimalis. Keamanan Maksimal.
  actions:
    - theme: brand
      text: Memulai
      link: /id/get-started/installation
    - theme: alt
      text: Referensi API
      link: /id/api-references

features:
  - title: Sihir Nol (Deterministik)
    details: Eksekusi instruksi bersifat linier dan sepenuhnya dapat diprediksi. VM beroperasi secara eksplisit, mengeksekusi instruksi tepat seperti yang didefinisikan.
  - title: Sadar akan Sumber Daya
    details: Dirancang dengan jejak memori minimal melalui penggunaan struktur data yang dioptimalkan seperti SmolStr dan ahash untuk manajemen metadata yang cepat.
  - title: Keamanan Eksplisit
    details: Keamanan dikelola melalui sistem Kapabilitas yang ketat. Setiap akses dan operasi VM harus memiliki izin yang secara eksplisit ditentukan oleh host sejak awal.
---

<HeroStats />