<script lang="ts">
  import auth from "../../authService";

  import { DataTable, DataTableSkeleton } from "carbon-components-svelte";
  import type { DataTableRow } from "carbon-components-svelte/types/DataTable/DataTable.svelte";
  const data = getData();
  async function getData() {
    const token = await auth.auth0.getTokenSilently();

    return fetch("http://localhost:3000/order", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }).then((res) => res.json());
  }

  function handleClick(row: CustomEvent<DataTableRow>) {
    window.location.hash = "#/order/details/" + row.detail.id;
  }
</script>

<h1>ORDERS</h1>

{#await data}
  <DataTableSkeleton />
{:then data}
  <div class="orders">
    <div class="order" />
    <DataTable
      expandable
      on:click:row={handleClick}
      sortable
      headers={[
        { key: "id", value: "Id" },
        { key: "order_nr", value: "OrderNumber" },
        { key: "title", value: "Title" },
        { key: "status", value: "Status" },
      ]}
      rows={data}
    >
      <svelte:fragment slot="expanded-row" let:row>
        <p>{data.find((x) => x.id == row.id).note}</p>
      </svelte:fragment>
    </DataTable>
  </div>
{:catch error}
  <h4>Error Loading Orders</h4>
{/await}
