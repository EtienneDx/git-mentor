import { render, screen } from "@testing-library/react";
import Router from "../routing/Router";

describe("App Component", () => {
  it("renders without crashing", () => {
    render(<Router />);
  });

  it("renders the Authentication component", () => {
    render(<Router />);
    expect(screen.getByText(/login/i)).toBeInTheDocument();
  });
});
