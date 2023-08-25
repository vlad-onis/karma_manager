<script>
    import { invoke } from '@tauri-apps/api'

    let username = '';
    let password = '';

    async function login() {
        console.log(username);
        console.log(password);
    }
</script>


<form on:submit|preventDefault={login}>
    <label>
      Username:
      <input type="text" bind:value={username} />
    </label>
    <label>
      Password:
      <input type="password" bind:value={password} />
    </label>
    <button type="submit">Login</button>
  </form>
  
  <style>
    form {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;
      max-width: 300px;
      margin: 0 auto;
    }
  </style>