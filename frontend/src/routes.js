import Home from "./routes/Home.svelte";
import NotFound from "./routes/NotFound.svelte";
import Order from "./routes/order/Order.svelte";
import OrderDetails from "./routes/order/OrderDetails.svelte";
import CreateOrder from "./routes/order/CreateOrder.svelte";
import Template from "./routes/Template.svelte";

const routes = {
  "/": Home,
  "/order": Order,
  "/order/details/:id": OrderDetails,
  "/order/create": CreateOrder,
  "/template": Template,
  "*": NotFound,
};

export { routes };
