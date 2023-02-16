<script lang="ts">
  import auth from "../authService";
  import {
    Grid,
    Row,
    Column,
    StructuredList,
    StructuredListHead,
    StructuredListCell,
    StructuredListRow,
    StructuredListBody,
  } from "carbon-components-svelte";
  const taskTemplates = getTaskTemplates();
  async function getTaskTemplates() {
    const token = await auth.auth0.getTokenSilently();

    return fetch("http://localhost:3000/template/task", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }).then((res) => res.json());
  }
</script>

<h1>TEMPLATES</h1>
{#await taskTemplates}
  <h4>Loading Templates</h4>
{:then data}
  <Grid>
    <Row>
      <Column aspectRatio="2x1">
        <h3>Task Templates</h3>
        <StructuredList>
          <StructuredListHead>
            <StructuredListRow>
              <StructuredListCell head>Id</StructuredListCell>
              <StructuredListCell head>Title</StructuredListCell>
            </StructuredListRow>
          </StructuredListHead>
          <StructuredListBody>
            {#each data as template}
              <StructuredListRow>
                <StructuredListCell noWrap>{template.id}</StructuredListCell>
                <StructuredListCell>{template.title}</StructuredListCell>
              </StructuredListRow>
            {/each}
          </StructuredListBody>
        </StructuredList>
      </Column>
      <Column aspectRatio="2x1">2x1</Column>
    </Row>
  </Grid>
{:catch error}
  <h4>Error Loading Templaplates</h4>
{/await}
