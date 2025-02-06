import { router } from "@/router";
import "./index.css";
import { RouterProvider } from "react-router-dom";
import { ThemeProvider } from "@/context/theme";
import { Toaster } from "@/components/ui/sonner";

function App() {
  return (
    <main className="flex h-screen w-screen flex-col bg-background">
      <ThemeProvider>
        <RouterProvider router={router} />
      </ThemeProvider>
      <Toaster />
    </main>
  );
}

export default App;
