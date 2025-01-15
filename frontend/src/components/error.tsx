import { Button } from "@/components/ui/button";
import { isRouteErrorResponse, useRouteError } from "react-router-dom";

export const RouteError = () => {
  const error = useRouteError();
  let errorMessage: string;

  if (isRouteErrorResponse(error)) {
    errorMessage = error.data;
  } else if (error instanceof Error) {
    errorMessage = error.message;
  } else if (typeof error === "string") {
    errorMessage = error;
  } else {
    console.error(error);
    errorMessage = "Unknown error";
  }

  return (
    <div
      className="flex h-screen w-screen flex-col items-center justify-center space-y-5"
      role="alert"
    >
      <h2 className="text-lg font-semibold text-red-500">
        Sorry, there seems to be some problems
      </h2>
      <h3 className="text-base font-semibold text-red-500">
        {errorMessage}
      </h3>
      <Button variant="destructive" onClick={() => window.location.assign(window.location.origin)}>
        Refresh
      </Button>
    </div>
  );
};
