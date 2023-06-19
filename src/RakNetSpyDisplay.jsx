const isNumber = number => !isNaN(parseInt(number))

const customRenderers = {
    "address,port,version": (name, data) => {
        if (isNumber(name)) {
            return <div class="raknet-inspector-pair">
                <p class="raknet-inspector-single-value">{data.address.join(".")}:{data.port}</p>
            </div>
        } else {
            return <div class="raknet-inspector-pair">
                <p class="raknet-inspector-pair-key">{name}</p>
                <p class="raknet-inspector-pair-value">{data.address.join(".")}:{data.port}</p>
            </div>
        }
    }
}

function renderData( dest, packet ) {
    const validKeys = Object
        .keys( packet )
        .filter( 
            key => 
                key != "id" && 
                key != "len" &&
                packet[ key ] != null
        );

    const complexValues = [];
    const simpleValues  = [];

    for (let key of validKeys) {
        if (typeof packet[key] == "object") {
            complexValues.push( key );
        } else {
            simpleValues.push( key );
        }
    }
    
    for (let key of simpleValues) {
        let value = packet[ key ];

        if (isNumber(key)) {
            dest.push(
                <div class="raknet-inspector-pair">
                    <p class="raknet-inspector-single-value">{ value.toString() }</p>
                </div>
            )
        } else {
            dest.push(
                <div class="raknet-inspector-pair">
                    <p class="raknet-inspector-pair-key">{ key }</p>
                    <p class="raknet-inspector-pair-value">{ value.toString() }</p>
                </div>
            )
        }
    }

    for (let key of complexValues) {
        const value = packet[ key ];

        if (typeof value == "object") {
            const keyHash = Object.keys(value).sort().join(",");

            const customRenderer = customRenderers[ keyHash ];
            if (customRenderer) {
                dest.push( 
                    customRenderer( key, value ) 
                )

                continue;
            }
        }

        if (!isNumber(key)) {
            dest.push(
                <p class="raknet-inspector-title">{ key }</p>
            );    
        }

        renderData( dest, packet[ key ] );
    }

    return dest;
}

export default function renderPacket( row ) {
    const { packet } = row;

    const newActivePacket = [
        <div class="raknet-inspector-pair">
            <p class="raknet-inspector-pair-key">ID</p>
            <p class="raknet-inspector-pair-value">{ row.id }</p>
        </div>,
        <div class="raknet-inspector-pair">
            <p class="raknet-inspector-pair-key">Name</p>
            <p class="raknet-inspector-pair-value">{ row.name }</p>
        </div>,
        <div class="raknet-inspector-pair">
            <p class="raknet-inspector-pair-key">Source</p>
            <p class="raknet-inspector-pair-value">{ row.client }</p>
        </div>,
        <div class="raknet-inspector-pair">
            <p class="raknet-inspector-pair-key">Type</p>
            <p class="raknet-inspector-pair-value">{ row.packetType }</p>
        </div>,

        <p class="raknet-inspector-title">Packet Data</p>
    ];

    const renderedPacket = renderData( [], packet );
    return [ ...newActivePacket, ...renderedPacket ];
}