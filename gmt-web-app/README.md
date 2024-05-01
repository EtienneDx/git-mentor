# GMT Web App

This project holds the frontend of the Git Mentor project.

## Available Scripts

In the project directory, you can run:

### `npm start`

To function, the app needs the backend to be up and running (see [README](../gmt-api/README.md) of gmt-api), and the corresponding sdk to be generated to serve as the api interface (see section below [make-sdk](#npm-run-make-sdk))

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

### `npm test`

Launches the test runner in the interactive watch mode.\
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `npm run build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

### `npm run lint` or `npm run lint:fix`

Runs eslint on all typescript files to signal or fix linting.

### `npm run format` or `npm run format:check`

Formats all of the files using Prettier.

### `npm run make-sdk`

Generates the SDK from the OpenAPI spec file. The SDK is generated in the `src/gmt-api` folder. This script requires the (gmt-api)[../gmt-api] project to be built.

Complete workflow:

```bash
cd ../gmt-api
cargo build
cd ../gmt-web-app
npm run make-sdk
```

To use the sdk, you can import it in your code:

```tsx
import { useApi } from "../../../context/api";

const MyComponent: React.FC = () => {
  const api = useApi();

  const buttonClicked = () => {
    const me = await api.getHelloMe();

    console.log(me);
  }

  return <></>;
}
```

See [the generated source](./src/gmt-api/) for more infos (Require local clone).

## Project Description

This project is a React application that serves as a web interface for Git Mentor. It is bootstrapped with Create React App and uses a set of common libraries to enhance its functionality such as:

- `React` and `React-DOM`: These are the core libraries that power our application.
- `TypeScript`: It helps us catch errors early in the development process and enhances our development experience with features like autocompletion, type inference, and type checking. It also makes our code more readable and self-documenting.
- `React Router DOM`: This library allows us to manage routing.
- `Axios`: We use this library to make HTTP requests to our Rust backend.
- `Tailwind CSS`: This library allows us to style our React components, enabling simple and compact styling.
- `Jest` (via @types/jest): This is our testing framework, which allows us to write unit and integration tests for our components.
- `ESLint` and `Prettier`: These tools help us maintain a consistent code style and catch potential issues early.

We chose not to use `Next.js` for this project because our current needs are well-served by Create React App. Moreover, as the frontend is served by an S3 bucket, the additional features provided by `Next.js` such as server-side rendering and static site generation would have been useless.
