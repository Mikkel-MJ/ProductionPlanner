import { createAuth0Client } from "@auth0/auth0-spa-js";
import { user, isAuthenticated, popupOpen } from "./store";

//with async/await
const auth0 = await createAuth0Client({
  domain: "dev-o5vpiij8.eu.auth0.com",
  clientId: "kmPTTfs02Wpmi6yyBPwRRzbMkOdRMrhT",
  authorizationParams: {
    redirect_uri: "http://127.0.0.1:5173/",
    audience: "prod-plan-api",
  },
});

async function login() {
  popupOpen.set(true);
  try {
    await auth0.loginWithPopup();

    user.set(await auth0.getUser());
    isAuthenticated.set(true);
  } catch (e) {
    console.error(e);
  } finally {
    popupOpen.set(false);
  }
}

function logout() {
  auth0.logout();
}

export default { auth0, login, logout };
