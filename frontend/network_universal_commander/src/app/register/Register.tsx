// src/app/register/Register.tsx
import { useState } from 'react';
import styles from './Register.module.css';
import Link from 'next/link';

const Register = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [passwordError, setPasswordError] = useState('');
  const [isValid, setIsValid] = useState(true);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    // Простейшая проверка пароля
    if (password.length < 8) {
      setPasswordError('Должен быть не короче 8 символов');
      setIsValid(false);
    } else {
      setPasswordError('');
      setIsValid(true);
      // Здесь может быть логика для отправки данных на сервер
      console.log('Регистрация:', { email, password });
    }
  };

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>Регистрация</h1>
      <form className={styles.form} onSubmit={handleSubmit}>
        <label className={styles.label} htmlFor="email">Введите email</label>
        <input
          className={styles.input}
          type="email"
          id="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        
        <label className={styles.label} htmlFor="password">Пароль</label>
        <input
          className={`${styles.input} ${!isValid && passwordError ? styles.error : ''}`}
          type="password"
          id="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
        { !isValid && passwordError && (
          <div className={styles.errorMessage}>{passwordError}</div>
        )}

        <button type="submit" className={styles.button}>Зарегистрироваться</button>
        <Link href="/login" className={styles.link}>Уже есть аккаунт? Войти</Link>
      </form>
    </div>
  );
};

export default Register;
