import { Link } from "react-router-dom";
import Box from "../../components/Box";
import LoginForm from "./components/LoginForm";

const Login: React.FC = () => {
  return (
    <div className="flex flex-col flex-wrap content-center justify-center h-screen w-full gap-2 bg-blue-300">
      <Box>
        <h1 className="font-bol</div>d text-2xl">Git-mentor</h1>
        <LoginForm />
        <Link to="/signup">Don't have an account? Sign up here</Link>
      </Box>
    </div>
  );
};

export default Login;
