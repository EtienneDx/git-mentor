import React from 'react';
import Footer from '../molecules/Footer';
import Navbar from '../molecules/Navbar';

const PageLayout: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <div className="flex flex-col min-h-screen">
      <Navbar />

      <main className="flex-grow">{children}</main>

      <Footer />
    </div>
  );
};

export default PageLayout;