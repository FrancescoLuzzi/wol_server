import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation } from "@tanstack/react-query";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { apiClient } from "@/lib/axios";
import { useEffect, useState } from "react";

const passwordRequirements = {
  minLength: 8,
  requiresUppercase: /[A-Z]/,
  requiresLowercase: /[a-z]/,
  requiresNumber: /[0-9]/,
  requiresSpecial: /[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/,
};

const signupSchema = z
  .object({
    full_name: z
      .string()
      .min(2, "Name must be at least 2 characters")
      .max(50, "Name must be less than 50 characters"),
    username: z
      .string()
      .min(3, "Username must be at least 3 characters")
      .max(20, "Username must be less than 20 characters")
      .regex(/^[a-zA-Z0-9_.]+$/, "Invalid characters in username"),
    email: z.string().email("Invalid email address"),
    password: z
      .string()
      .min(
        passwordRequirements.minLength,
        `Password must be at least ${passwordRequirements.minLength} characters`,
      )
      .regex(
        passwordRequirements.requiresUppercase,
        "Requires at least one uppercase letter",
      )
      .regex(
        passwordRequirements.requiresLowercase,
        "Requires at least one lowercase letter",
      )
      .regex(
        passwordRequirements.requiresNumber,
        "Requires at least one number",
      )
      .regex(
        passwordRequirements.requiresSpecial,
        "Requires at least one special character",
      ),
    confirmPassword: z.string(),
    request_text: z
      .string()
      .min(10, "Request text must be at least 10 characters")
      .max(500, "Request text must be less than 500 characters"),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
  });

export function SignupForm() {
  const form = useForm<z.infer<typeof signupSchema>>({
    resolver: zodResolver(signupSchema),
    defaultValues: {
      full_name: "",
      username: "",
      email: "",
      password: "",
      confirmPassword: "",
      request_text: "",
    },
  });

  const [passwordChecks, setPasswordChecks] = useState({
    minLength: false,
    hasUppercase: false,
    hasLowercase: false,
    hasNumber: false,
    hasSpecial: false,
  });

  const { mutate: signup, isPending } = useMutation({
    mutationFn: async (values: z.infer<typeof signupSchema>) => {
      const { data } = await apiClient.post("/auth/signup", values, {
        headers: { "content-type": "application/x-www-form-urlencoded" },
      });
      return data;
    },
    onSuccess: () => {
      window.location.href = "/auth/login";
    },
    onError: (error) => {
      form.setError("root", { message: error.message });
    },
  });

  // Password strength checker
  useEffect(() => {
    const password = form.watch("password");
    setPasswordChecks({
      minLength: password.length >= passwordRequirements.minLength,
      hasUppercase: passwordRequirements.requiresUppercase.test(password),
      hasLowercase: passwordRequirements.requiresLowercase.test(password),
      hasNumber: passwordRequirements.requiresNumber.test(password),
      hasSpecial: passwordRequirements.requiresSpecial.test(password),
    });
  }, [form.watch("password")]);

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit((values) => signup(values))}
        className="space-y-6"
      >
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <FormField
            control={form.control}
            name="full_name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Full Name</FormLabel>
                <FormControl>
                  <Input placeholder="John Doe" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input placeholder="johndoe123" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input placeholder="email@example.com" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" placeholder="Password" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="confirmPassword"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Confirm Password</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="Confirm Password"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        {/* Password Requirements */}
        <div className="space-y-2">
          <p className="text-sm font-medium">Password must contain:</p>
          <ul className="grid grid-cols-2 gap-2 text-sm text-muted-foreground">
            <li
              className={`flex items-center ${passwordChecks.minLength ? "text-green-500" : ""}`}
            >
              {passwordChecks.minLength ? "✓" : "•"} At least{" "}
              {passwordRequirements.minLength} characters
            </li>
            <li
              className={`flex items-center ${passwordChecks.hasUppercase ? "text-green-500" : ""}`}
            >
              {passwordChecks.hasUppercase ? "✓" : "•"} One uppercase letter
            </li>
            <li
              className={`flex items-center ${passwordChecks.hasLowercase ? "text-green-500" : ""}`}
            >
              {passwordChecks.hasLowercase ? "✓" : "•"} One lowercase letter
            </li>
            <li
              className={`flex items-center ${passwordChecks.hasNumber ? "text-green-500" : ""}`}
            >
              {passwordChecks.hasNumber ? "✓" : "•"} One number
            </li>
            <li
              className={`flex items-center ${passwordChecks.hasSpecial ? "text-green-500" : ""}`}
            >
              {passwordChecks.hasSpecial ? "✓" : "•"} One special character
            </li>
          </ul>
        </div>

        <FormField
          control={form.control}
          name="request_text"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Request Text</FormLabel>
              <FormControl>
                <Textarea
                  placeholder="Please describe your reason for signing up..."
                  className="min-h-[100px]"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        {form.formState.errors.root && (
          <p className="text-sm font-medium text-destructive">
            {form.formState.errors.root.message}
          </p>
        )}

        <Button type="submit" className="w-full" disabled={isPending}>
          {isPending ? "Creating account..." : "Sign Up"}
        </Button>
      </form>
    </Form>
  );
}
