import { Link } from "react-router-dom";
import Box from "../../components/Box";
import SignupForm from "./components/SignupForm";

const Signup: React.FC = () => {
  return (
    <div className="flex flex-col flex-wrap content-center justify-center h-screen w-full gap-2 bg-blue-300">
      <Box>
        <h1 className="font-bold text-2xl">Git-mentor</h1>
        <SignupForm />
        <Link to="/login">Already have an account? Log in here</Link>
      </Box>
    </div>
  );
};

export default Signup;
