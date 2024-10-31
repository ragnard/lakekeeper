/**
 * iceberg-catalog
 * Implementation of the Iceberg REST Catalog server. 
 *
 * The version of the OpenAPI document: 0.4.2
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';
import { TabularType } from './tabularType';

export class DeletedTabularResponse {
    /**
    * Date when the tabular was created
    */
    'createdAt': Date;
    /**
    * Date when the tabular was deleted
    */
    'deletedAt': Date;
    /**
    * Date when the tabular will not be recoverable anymore
    */
    'expirationDate': Date;
    /**
    * Unique identifier of the tabular
    */
    'id': string;
    /**
    * Name of the tabular
    */
    'name': string;
    /**
    * List of namespace parts the tabular belongs to
    */
    'namespace': Array<string>;
    'typ': TabularType;
    /**
    * Warehouse ID where the tabular is stored
    */
    'warehouseId': string;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "createdAt",
            "baseName": "created_at",
            "type": "Date"
        },
        {
            "name": "deletedAt",
            "baseName": "deleted_at",
            "type": "Date"
        },
        {
            "name": "expirationDate",
            "baseName": "expiration_date",
            "type": "Date"
        },
        {
            "name": "id",
            "baseName": "id",
            "type": "string"
        },
        {
            "name": "name",
            "baseName": "name",
            "type": "string"
        },
        {
            "name": "namespace",
            "baseName": "namespace",
            "type": "Array<string>"
        },
        {
            "name": "typ",
            "baseName": "typ",
            "type": "TabularType"
        },
        {
            "name": "warehouseId",
            "baseName": "warehouse_id",
            "type": "string"
        }    ];

    static getAttributeTypeMap() {
        return DeletedTabularResponse.attributeTypeMap;
    }
}

export namespace DeletedTabularResponse {
}