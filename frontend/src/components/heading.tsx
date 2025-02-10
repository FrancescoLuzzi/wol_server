import { Link } from "react-router-dom";
import { useAuth } from "@/hooks/auth";
import { ThemeToggle } from "./theme";
import { UserSignupLogin } from "./signup-login";
import { UserNavigation } from "./user-navigation";

export function NavigationHeader() {
  const { ctx, logout } = useAuth();

  return (
    <nav className="flex items-center justify-between border-b px-6 py-4">
      <Link to="/" className="flex items-center gap-2">
        <span className="text-lg font-semibold">Your Brand</span>
      </Link>

      <div className="flex space-x-4">
        {!ctx ? (
          <UserSignupLogin />
        ) : (
          <UserNavigation ctx={ctx} logout={logout} />
        )}
        <ThemeToggle />
      </div>
    </nav>
  );
}
