import { router } from "@/router";
import "./index.css";
import { RouterProvider } from "react-router-dom";
import { ThemeProvider } from "@/context/theme";

function App() {
  return (
    <main className="h-screen w-screen bg-background">
      <ThemeProvider>
        <RouterProvider router={router} />
      </ThemeProvider>
    </main>
  );
}

export default App;
