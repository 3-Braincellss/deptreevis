import { createRoot } from "react-dom/client";
import { RouterProvider } from "react-router";
import "@mantine/core/styles.css";
import { createTheme, MantineProvider } from "@mantine/core";
import router from "./routes";

const theme = createTheme({});

createRoot(document.getElementById("root")!).render(
  <MantineProvider theme={theme}>
    <RouterProvider router={router} />
  </MantineProvider>,
);
