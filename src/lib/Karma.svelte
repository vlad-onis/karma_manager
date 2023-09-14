<script>
    import { invoke } from '@tauri-apps/api'

    let name = '';
    let purpose = '';
    let result = '';
    async function create_karma() {
        console.log(name);
        console.log(purpose);

      try {
        result = await invoke('create_karma', { name, purpose });
      } catch (err) {
        console.log(err);
      }
    }
</script>


<form on:submit|preventDefault={create_karma}>
    <label>
      Karma Name:
      <input type="text" bind:value={name} />
    </label>
    <label>
      Karma purpose:
      <input type="text" bind:value={purpose} />
    </label>
    <button type="submit">Create</button>
    <p>{result}</p>
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
