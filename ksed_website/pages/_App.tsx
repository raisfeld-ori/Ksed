//@ts-expect-error
import Layout from "@/app/layout";
import { AppProps } from "next/app";
const MyApp: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  );
}

export default MyApp;