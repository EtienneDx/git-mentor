import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import "@testing-library/jest-dom";
import axios from "axios";
import Authentication from "../features/auth/Authentication";

jest.mock("axios");

describe("Authentication Component", () => {
    it("renders without crashing", () => {
        render(<Authentication />);
    });

    it("submits the form with valid credentials and updates the token", async () => {
        const token = "fakeToken";
        const mockResponse = { data: { token } };

        (axios.post as jest.Mock).mockResolvedValueOnce(mockResponse);

        render(<Authentication />);

        fireEvent.change(screen.getByLabelText(/email/i), {
            target: { value: "test@example.com" },
        });
        fireEvent.change(screen.getByLabelText(/password/i), {
            target: { value: "password123" },
        });
        fireEvent.submit(screen.getByText(/login/i));

        await waitFor(() => {
            expect(axios.post).toHaveBeenCalledWith("/api/auth/login", {
                email: "test@example.com",
                password: "password123",
            });
            expect(screen.getByText(/authenticated/i)).toBeInTheDocument();
            expect(screen.getByText(/token/i)).toHaveTextContent(token);
        });
    });

    it("handles form submission error and displays an error message", async () => {
        const errorMessage = "Invalid email or password";
        (axios.post as jest.Mock).mockRejectedValueOnce(
            new Error(errorMessage)
        );

        render(<Authentication />);

        fireEvent.change(screen.getByLabelText(/email/i), {
            target: { value: "invalid@example.com" },
        });
        fireEvent.change(screen.getByLabelText(/password/i), {
            target: { value: "invalidpassword" },
        });
        fireEvent.submit(screen.getByText(/login/i));

        await waitFor(() => {
            expect(axios.post).toHaveBeenCalledWith("/api/auth/login", {
                email: "invalid@example.com",
                password: "invalidpassword",
            });
            expect(screen.getByText(errorMessage)).toBeInTheDocument();
        });
    });
});
