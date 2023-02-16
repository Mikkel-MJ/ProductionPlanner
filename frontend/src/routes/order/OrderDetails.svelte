<script lang="ts">
  import auth from "../../authService";
  import {
    Dropdown,
    Loading,
    TextArea,
    TextInput,
    ToastNotification,
    Button,
    ButtonSet,
    Modal,
  } from "carbon-components-svelte";
  export let params;
  let toast = false;
  let openDelete = false;

  let orderNr = null;
  let orderTitle = null;
  let orderNote = null;
  let status = null;
  const data = getData();
  async function getData() {
    const token = await auth.auth0.getTokenSilently();

    return fetch("http://localhost:3000/order/" + params.id, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })
      .then((res) => res.json())
      .then((data) => {
        orderNote = data.note;
        orderNr = data.order_nr;
        orderTitle = data.title;
        status = data.status;
        return data;
      })
      .catch((err) => {
        console.log(err);
      });
  }

  async function saveChanges() {
    const token = await auth.auth0.getTokenSilently();

    const res = await fetch("http://localhost:3000/order/" + params.id, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        order_nr: orderNr,
        title: orderTitle,
        note: orderNote,
        status: status,
      }),
    })
      .then((res) => {
        toast = true;
      })
      .catch((err) => {
        console.log(err);
      });
  }
  async function DeleteOrder() {
    const token = await auth.auth0.getTokenSilently();

    const res = await fetch("http://localhost:3000/order/" + params.id, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    })
      .then((res) => {
        toast = true;
      })
      .catch((err) => {
        console.log(err);
      });
  }
</script>

<div class="body">
  <h1>DETAILS For Order:{params.id}</h1>
  {#await data}
    <Loading />
  {:then data}
    <TextInput
      labelText="Order Number"
      helperText="Enter the order number for the order you want to create"
      placeholder="Order Number"
      bind:value={orderNr}
    />
    <TextInput
      labelText="Order Title"
      helperText="Enter the title of the order for better identification of the order"
      placeholder="Order Title"
      bind:value={orderTitle}
    />
    <TextArea
      labelText="Order Note"
      placeholder="Enter Notes for the order..."
      bind:value={orderNote}
    />
    <Dropdown
      titleText="Status"
      bind:selectedId={status}
      items={[
        { id: "Open", text: "Open" },
        { id: "Completed", text: "Completed" },
        { id: "Blocked", text: "Blocked" },
      ]}
    />
    <div class="buttons">
      <ButtonSet>
        <Button on:click={saveChanges}>Update Order</Button>
        <Button kind="danger" on:click={() => (openDelete = true)}
          >Delete Order</Button
        >
      </ButtonSet>
    </div>
  {:catch error}
    <h4>Error Loading Order</h4>
  {/await}

  {#if toast}
    <ToastNotification
      kind="success"
      title="Order Updated"
      subtitle="Order was Updated successfully"
      caption={new Date().toLocaleString()}
      timeout={2000}
      on:close={() => (toast = false)}
    />
  {/if}

  <Modal
    danger
    bind:open={openDelete}
    modalHeading="Delete Order"
    primaryButtonText="Delete"
    secondaryButtonText="Cancel"
    on:click:button--secondary={() => (openDelete = false)}
    on:open
    on:close
    on:submit={() => {
      openDelete = false;
      DeleteOrder();
      history.back();
    }}
  >
    <p>This is a permanent action and cannot be undone.</p>
  </Modal>
</div>

<style>
  .body {
    padding: 1rem;
    margin-top: 1rem;
  }
  .buttons {
    margin-top: 1rem;
  }
</style>
