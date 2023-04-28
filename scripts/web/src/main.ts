import './style.css'
import {createConnectTransport, createPromiseClient, Interceptor} from "@bufbuild/connect-web";
import {ObjectApi} from "../sdk/service_connectweb.ts";

const header_interceptor: Interceptor = (next) => async (req) => {
    req.header.set("content-type", "application/grpc-web");
    return await next(req);
};

const transport = createConnectTransport({
                                             baseUrl: "http://localhost:3000",
                                             interceptors: [header_interceptor],
                                         });


const client = createPromiseClient(ObjectApi, transport);

async function getObjects() {
    const objects = await client.list_objects({});
    return objects;
}

getObjects().then(objects => {
    document.querySelector<HTMLDivElement>('#app')!.innerHTML = objects.objects.join("<br/>");
});
