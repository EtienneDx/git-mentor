import React from 'react';
import { render, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import axios from 'axios';
import Authentication from '../features/auth/Authentication';

jest.mock('axios');

describe('Authentication Component', () => {
  it('renders without crashing', () => {
    render(<Authentication />);
  });

  it('submits the form with valid credentials and updates the token', async () => {
    const token = 'fakeToken';
    const mockResponse = { data: { token } };

    (axios.post as jest.Mock).mockResolvedValueOnce(mockResponse);

    const { getByLabelText, getByText } = render(<Authentication />);

    fireEvent.change(getByLabelText(/email/i), { target: { value: 'test@example.com' } });
    fireEvent.change(getByLabelText(/password/i), { target: { value: 'password123' } });
    fireEvent.submit(getByText(/login/i));

    await waitFor(() => {
      expect(axios.post).toHaveBeenCalledWith('/api/auth/login', {
        email: 'test@example.com',
        password: 'password123',
      });
      expect(getByText(/authenticated/i)).toBeInTheDocument();
      expect(getByText(/token/i)).toHaveTextContent(token);
    });
  });

  it('handles form submission error and displays an error message', async () => {
    const errorMessage = 'Invalid email or password';
    (axios.post as jest.Mock).mockRejectedValueOnce(new Error(errorMessage));

    const { getByLabelText, getByText } = render(<Authentication />);

    fireEvent.change(getByLabelText(/email/i), { target: { value: 'invalid@example.com' } });
    fireEvent.change(getByLabelText(/password/i), { target: { value: 'invalidpassword' } });
    fireEvent.submit(getByText(/login/i));

    await waitFor(() => {
      expect(axios.post).toHaveBeenCalledWith('/api/auth/login', {
        email: 'invalid@example.com',
        password: 'invalidpassword',
      });
      expect(getByText(errorMessage)).toBeInTheDocument();
    });
  });
});