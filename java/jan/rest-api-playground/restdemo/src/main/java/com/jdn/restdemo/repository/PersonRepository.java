package com.jdn.restdemo.repository;

import com.jdn.restdemo.model.Person;
import org.springframework.data.repository.PagingAndSortingRepository;

public interface PersonRepository extends PagingAndSortingRepository<Person,Integer> {

}
