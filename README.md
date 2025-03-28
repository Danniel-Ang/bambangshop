# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

Menurut saya, dari berbagai contoh observer pattern yang saya lihat, Subscriber biasanya dibuat dalam bentuk interface karena Subscriber yang berbeda dapat mempunyai prilaku yang berbeda. Namun pada kasus ini, say rasa cukup dengan satu tipe subscriber sudah cukup karena itu akan membuat semuanya menjadi lebih simpel dibanding kita menyiapkan beberapa tipe berbeda.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

Vec mungkin dapat bekerja jika kita hanya mau menyimpan beberapa subscriber saja dan kita tidak terlalu memungkiri dengan kemungkinan pencarian subscribers. Namun karena url harus unik. Menggunakan map/dictionry menurut saya lebih rasional karena lebih cepat dalam melakukan pencarian, penambahan ataupun penghapusan url.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

Karena kita ingin menjalankan concurrency dengan aman, menurut saya menggunakan Singleton pattern saja tidak cukup karena tidak secara langsung menangani masalah akses bersamaan. Jadi, memakai DashMap lebih masuk akal karena lebih simpel dibandingkan harus membuat mekanisme locking sendiri, yang bisa jadi lebih kompleks dan rawan kesalahan.

#### Reflection Publisher-2

1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

Dalam MVC, Model seringkali menangani data dan logika bisnis sekaligus. Namun, dengan memisahkan Service dan Repository, kita bisa mengisolasi logika bisnis dan akses data secara terpisah. Hal ini membuat kode lebih modular, mudah di-maintain, dan lebih mudah diuji karena masing-masing komponen memiliki tanggung jawab yang jelas.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

Kalau kita hanya menggunakan Model, maka Model tersebut harus menangani segala hal mulai dari interaksi dengan database hingga logika bisnis. Misalnya, model Program, Subscriber, dan Notification akan saling bergantung dan berinteraksi secara langsung, yang dapat menyebabkan kode menjadi rumit dan susah di-maintain. Pisah-pisah tugas ini membantu mengurangi kompleksitas dan memudahkan perbaikan atau penambahan fitur baru.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

Saya sudah mencoba Postman dan menurut saya alat ini sangat membantu dalam pengujian API. Postman memudahkan saya untuk mengirim request dengan berbagai metode (GET, POST, dll), melihat response secara real time, serta melakukan automatisasi pengujian dengan collection dan environment. Fitur-fitur tersebut sangat berguna untuk proyek kelompok atau pengembangan perangkat lunak di masa depan.

#### Reflection Publisher-3

