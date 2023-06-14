# ProductionPlanner

Rust backend using Axum framework and sqlx(postgresql)

Svelte spa frontend 


steps to run:
1. go to backend folder: cd backend
2. run: cargo sqlx prepare
3. check that debug_set_env_var() in main line 22 is commented out(only run this when running backend local);
4. run docker compose up from root folder.
