import styles from './styles.module.css';

export default function Navbar() {
  return (
    <div className={styles.nav}>
      <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/add">Add a coin</a></li>
      </ul>
    </div>
  );
}
