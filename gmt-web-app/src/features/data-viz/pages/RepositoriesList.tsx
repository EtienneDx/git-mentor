import { useState, useEffect } from "react";
import RepositoriesAPI from "../services/repositoriesAPI";
import { Repository } from "../../../helpers/types";

const RepositoriesList = () => {
  const [repositories, setRepositories] = useState<Repository[]>([]);

  useEffect(() => {
    const loadRepositories = async () => {
      const newRepositories: Repository[] = await RepositoriesAPI.fetch();
      setRepositories(newRepositories);
    };
    loadRepositories();
  }, []);

  return (
    <div>
      <h1>My Repositories</h1>
      {repositories.length === 0 ? (
        <h1>Loading Repositories...</h1>
      ) : (
        <div>
          {repositories.map((repository) => (
            <div key={repository.id}>{repository.name}</div>
          ))}
        </div>
      )}
    </div>
  );
};

export default RepositoriesList;
