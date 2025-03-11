import { createBrowserRouter, RouteObject } from "react-router";
import Home from "./pages/home";

const homeRoute = (): RouteObject => ({
  path: "/",
  Component: Home,
});

const router = createBrowserRouter([homeRoute()]);

export default router;
