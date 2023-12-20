import Box from "../../components/Box";
import AuthenticationForm from "./components/AuthenticationForm";

const Authentication: React.FC = () => {
  return (
    <div className="flex flex-col flex-wrap content-center justify-center h-screen w-full gap-2 bg-blue-300">
      <Box>
        <h1 className="font-bold text-2xl">Email-Password Authentication</h1>
        <AuthenticationForm />
      </Box>
    </div>
  );
};

export default Authentication;
