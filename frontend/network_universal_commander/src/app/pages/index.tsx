'use client';
// src/pages/index.tsx
import Link from 'next/link';

export default function Start() {
  return (
    <div>
      <h1>Главная страница</h1>
      <nav>
        <Link href="/login">Вход</Link>
        <Link href="/register">Регистрация</Link>
      </nav>
    </div>
  );
}
