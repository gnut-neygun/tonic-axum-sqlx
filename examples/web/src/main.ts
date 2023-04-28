import './style.css'
import {createGrpcWebTransport, createPromiseClient} from "@bufbuild/connect-web";
import {ObjectApi} from "../sdk/service_connectweb.ts";


const transport = createGrpcWebTransport({
                                             baseUrl: "http://localhost:3000",
                                         });


const client = createPromiseClient(ObjectApi, transport);

async function getObjects() {
    const objects = await client.list_objects({});
    console.log(objects);
    return objects;
}

getObjects().then(objects => {
    document.querySelector<HTMLDivElement>('#app')!.innerHTML = objects.objects.map(o => o.toJsonString()).join("<br/>");
});
