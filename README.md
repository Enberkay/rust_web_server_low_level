# Rust Low-Level HTTP Server (No External Library)

โปรเจกต์ตัวอย่างเขียน HTTP Server ด้วย Rust โดยใช้แค่ Standard Library (`std::net` และ `std::io`) แบบ low-level เพื่อเรียนรู้พื้นฐานของ TCP socket และ HTTP protocol

---

## คุณสมบัติ

- รับ HTTP Request และส่ง HTTP Response แบบง่าย
- Routing แบบ basic รวมถึง
  - Static routes: `/`, `/hello`, `/api`
  - Dynamic route แบบ parameter: `/user/:id`
- ตรวจสอบรูปแบบ `user_id` ว่าเป็นตัวเลขเท่านั้น
- ส่งสถานะ HTTP เช่น 200 OK, 400 Bad Request, 404 Not Found

---

## วิธีใช้งาน

1. คอมไพล์และรันเซิร์ฟเวอร์

```bash
rustc server.rs
./server


curl http://localhost:7878

# ทดสอบ routing
curl http://localhost:7878/
curl http://localhost:7878/hello
curl http://localhost:7878/api
curl http://localhost:7878/unknown

# ทดสอบ dynamic route /user/:id
curl http://localhost:7878/user/123      # ควรได้ข้อความ "You requested user 123"
curl http://localhost:7878/user/abc      # ได้ HTTP 400 Bad Request
curl http://localhost:7878/user/          # ได้ HTTP 400 หรือ 404 Not Found
