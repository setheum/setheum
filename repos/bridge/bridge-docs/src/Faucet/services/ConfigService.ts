import axios from "axios";

export async function getDomains(root) {
    var resp = await axios({
        url: root + "/domains",
        method: "GET"
    })

    if(resp.status == 200) {
        return resp.data;
    } else {
        return [];
    }
}

export async function getTokens(root, domainID) {
    var resp = await axios({
        url: root + "/domains/" + domainID+ "/resources",
        method: "GET"
    })

    if(resp.status == 200) {
        return resp.data;
    } else {
        return [];
    }
}

export async function mintRequest(root, domain, resourceId, recipient){
    var resp = await axios({
        url: root + "/domains/" + domain + "/resources/" + resourceId+"/drip",
        headers: {
            'Content-Type': 'application/json'
        },
        data: {
            recipient: recipient
        }, 
        method: "POST"
    })
    if(resp.status == 200) {
        return resp.data;
    } else {
        console.log(resp.status + " mint request failed: " + resp.data)
        return {};
    }
}