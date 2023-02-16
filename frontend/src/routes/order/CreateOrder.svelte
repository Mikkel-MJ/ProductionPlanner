<script lang="ts">
  import auth from "../../authService";
  import {
    Button,
    TextArea,
    TextInput,
    ToastNotification,
  } from "carbon-components-svelte";
  let toast = false;
  let orderNr = null;
  let orderTitle = null;
  let orderNote = null;
  function print() {
    console.log(orderNr);
  }

  async function createOrder() {
    const token = await auth.auth0.getTokenSilently();

    const res = await fetch("http://localhost:3000/order", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        order_nr: orderNr,
        title: orderTitle,
        note: orderNote,
      }),
    })
      .then((res) => {
        if (res.status === 200) toast = true;
      })
      .catch((err) => {
        console.log(err);
      });
  }
</script>

{#if toast}
  <ToastNotification
    kind="success"
    title="Order Created"
    subtitle="Order was created successfully"
    caption={new Date().toLocaleString()}
    timeout={3000}
    on:close={() => (toast = false)}
  />
{/if}

<h1>Create Order</h1>
<p>Insert the data to create order</p>
<div>
  <div class="input">
    <TextInput
      labelText="Order Number"
      helperText="Enter the order number for the order you want to create"
      placeholder="Order Number"
      bind:value={orderNr}
    />
  </div>
  <div class="input">
    <TextInput
      labelText="Order Title"
      helperText="Enter the title of the order for better identification of the order"
      placeholder="Order Title"
      bind:value={orderTitle}
    />
  </div>
</div>

<div class="note">
  <TextArea
    labelText="Order Note"
    placeholder="Enter Notes for the order..."
    bind:value={orderNote}
  />
</div>
<div class="create-button">
  <Button on:click={createOrder}>Create Order</Button>
</div>

<style>
  .note {
    padding-top: 1rem;
    padding: 1rem;
  }

  .input {
    padding: 1rem;
  }
  .create-button {
    padding: 1rem;
  }
</style>
