import React from 'react';
import { Link } from 'react-router-dom';
import { useTokenData } from '../../context/authentication';
import { GitMentorLogo } from '../atoms/git-mentor';

const Navbar: React.FC = () => {
  const { user_id } = useTokenData();
  return (
    <nav className="bg-gray-800">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <Link to="https://github.com/EtienneDx/git-mentor"><GitMentorLogo /></Link>
            </div>
            <div className="hidden md:block">
              <div className="ml-10 flex items-baseline space-x-4 text-white">
                <Link to={`/${user_id}`}>Home</Link>
                <Link to={`/${user_id}/groups`}>My Groups</Link>
                <Link to={`/${user_id}/repositories`}>My Repositories</Link>
                <Link to="/students">All Students</Link>
                <Link to="/signout">Signout</Link>
              </div>
            </div>
          </div>
          <div className="-mr-2 flex md:hidden">
            {/* Add your mobile menu button here */}
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;